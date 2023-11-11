module Main exposing (..)

import Browser
import Html
import Html.Attributes as HtmlAttr
import Html.Events as HtmlEv



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    String


init : Model
init =
    ""



-- UPDATE


type Msg
    = NewName String


update : Msg -> Model -> Model
update msg model =
    case msg of
        NewName name ->
            name



-- VIEW


view : Model -> Html.Html Msg
view name =
    Html.div []
        [ Html.input
            [ HtmlAttr.placeholder "enter your name"
            , HtmlAttr.value name
            , HtmlEv.onInput NewName
            ]
            []
        , Html.node "wasm-wrapper"
            [ HtmlAttr.attribute "name" name
            , HtmlAttr.attribute "coll-a" "true"
            , HtmlAttr.attribute "coll-c" "true"
            ]
            []
        ]
